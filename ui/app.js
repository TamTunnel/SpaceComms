/**
 * SpaceComms Dashboard Application
 * Connects to a SpaceComms node and displays real-time data
 */

// Configuration
const CONFIG = {
  nodeUrl: window.location.origin, // Default to same origin
  constellationHubUrl: "http://localhost:9001", // Constellation Hub Mock
  refreshInterval: 5000, // 5 seconds
  maxRetries: 3,
};

// State
let state = {
  connected: false,
  health: null,
  peers: [],
  cdms: [],
  metrics: null,
  alerts: [],
  retryCount: 0,
};

// DOM Elements
const elements = {
  connectionStatus: document.getElementById("connection-status"),
  nodeId: document.getElementById("node-id"),
  healthStatus: document.getElementById("health-status"),
  healthUptime: document.getElementById("health-uptime"),
  healthPeers: document.getElementById("health-peers"),
  healthCdms: document.getElementById("health-cdms"),
  topologySvg: document.getElementById("topology-svg"),
  peersCount: document.getElementById("peers-count"),
  peersTable: document.getElementById("peers-table"),
  cdmsCount: document.getElementById("cdms-count"),
  cdmsTable: document.getElementById("cdms-table"),
  metricAnnounced: document.getElementById("metric-announced"),
  metricWithdrawn: document.getElementById("metric-withdrawn"),
  metricSent: document.getElementById("metric-sent"),
  metricErrors: document.getElementById("metric-errors"),
  alertsPanel: document.getElementById("alerts-panel"),
  alertsCount: document.getElementById("alerts-count"),
  alertsTable: document.getElementById("alerts-table"),
};

// API Functions
async function fetchHealth() {
  const response = await fetch(`${CONFIG.nodeUrl}/health`);
  if (!response.ok) throw new Error("Health check failed");
  return response.json();
}

async function fetchPeers() {
  const response = await fetch(`${CONFIG.nodeUrl}/peers`);
  if (!response.ok) throw new Error("Failed to fetch peers");
  return response.json();
}

async function fetchCdms() {
  const response = await fetch(`${CONFIG.nodeUrl}/cdms`);
  if (!response.ok) throw new Error("Failed to fetch CDMs");
  return response.json();
}

async function fetchMetrics() {
  try {
    const response = await fetch(`${CONFIG.nodeUrl}/metrics`);
    if (!response.ok) return null;
    return response.json();
  } catch {
    return null;
  }
}

async function fetchAlerts() {
  try {
    const response = await fetch(`${CONFIG.constellationHubUrl}/alerts`);
    if (!response.ok) return { alerts: [] };
    return response.json();
  } catch {
    return { alerts: [] };
  }
}

// UI Update Functions
function updateConnectionStatus(connected) {
  state.connected = connected;
  if (connected) {
    elements.connectionStatus.className = "status-indicator connected";
    elements.connectionStatus.textContent = "● Connected";
    state.retryCount = 0;
  } else {
    elements.connectionStatus.className = "status-indicator disconnected";
    elements.connectionStatus.textContent = "● Disconnected";
  }
}

function updateHealth(health) {
  state.health = health;

  // Status
  const statusEl = elements.healthStatus;
  statusEl.textContent = health.status || "--";
  statusEl.className = "health-value";
  if (health.status === "healthy") {
    statusEl.classList.add("healthy");
  } else if (health.status === "degraded") {
    statusEl.classList.add("degraded");
  } else {
    statusEl.classList.add("unhealthy");
  }

  // Uptime
  if (health.uptime_seconds !== undefined) {
    elements.healthUptime.textContent = formatUptime(health.uptime_seconds);
  }

  // Peers - handle both nested and flat formats
  const peerCount = health.peers?.connected ?? health.peer_count ?? "--";
  elements.healthPeers.textContent = peerCount;

  // CDMs
  elements.healthCdms.textContent =
    health.cdms_active ?? health.cdm_count ?? "--";

  // Node ID
  if (health.node_id) {
    elements.nodeId.textContent = `Node: ${health.node_id.slice(0, 8)}...`;
  }
}

function updatePeers(peersData) {
  const peers = peersData.peers || [];
  state.peers = peers;

  elements.peersCount.textContent = peers.length;

  const tbody = elements.peersTable.querySelector("tbody");

  if (peers.length === 0) {
    tbody.innerHTML =
      '<tr class="empty-row"><td colspan="4">No peers connected</td></tr>';
    return;
  }

  tbody.innerHTML = peers
    .map(
      (peer) => `
        <tr>
            <td><code>${peer.id ? peer.id.slice(0, 12) + "..." : "Unknown"}</code></td>
            <td>${peer.address || "--"}</td>
            <td><span class="status-badge ${getStatusClass(peer.status)}">${peer.status || "Unknown"}</span></td>
            <td>${peer.messages_received || 0} / ${peer.messages_sent || 0}</td>
        </tr>
    `,
    )
    .join("");
}

function updateCdms(cdmsData) {
  const cdms = cdmsData.cdms || [];
  state.cdms = cdms;

  elements.cdmsCount.textContent = cdms.length;

  const tbody = elements.cdmsTable.querySelector("tbody");

  if (cdms.length === 0) {
    tbody.innerHTML =
      '<tr class="empty-row"><td colspan="6">No CDMs available — <em>Inject one via Space-Track Mock</em></td></tr>';
    return;
  }

  tbody.innerHTML = cdms
    .map((cdm) => {
      const riskClass = getRiskClass(cdm.collision_probability);
      return `
            <tr>
                <td><code>${cdm.cdm_id ? cdm.cdm_id.slice(0, 16) : "--"}</code></td>
                <td>${cdm.object1_id || "--"}</td>
                <td>${cdm.object2_id || "--"}</td>
                <td>${formatDate(cdm.tca)}</td>
                <td>${cdm.miss_distance_m ? (cdm.miss_distance_m / 1000).toFixed(3) + " km" : "--"}</td>
                <td class="${riskClass}">${cdm.collision_probability ? formatProbability(cdm.collision_probability) : "--"}</td>
            </tr>
        `;
    })
    .join("");
}

function updateAlerts(alertsData) {
  const alerts = alertsData.alerts || [];
  state.alerts = alerts;

  // Show/hide alerts panel based on whether there are alerts
  if (elements.alertsPanel) {
    elements.alertsPanel.style.display = alerts.length > 0 ? "block" : "none";
  }

  if (elements.alertsCount) {
    elements.alertsCount.textContent =
      alertsData.unacknowledged || alerts.length;
  }

  if (!elements.alertsTable) return;

  const tbody = elements.alertsTable.querySelector("tbody");

  if (alerts.length === 0) {
    tbody.innerHTML =
      '<tr class="empty-row"><td colspan="6">No alerts</td></tr>';
    return;
  }

  tbody.innerHTML = alerts
    .map((alert) => {
      const severityClass = (alert.severity || "low").toLowerCase();
      return `
            <tr>
                <td><span class="severity-badge ${severityClass}">${alert.severity || "N/A"}</span></td>
                <td>${alert.satellite_name || "--"}</td>
                <td><code>${alert.cdm_id ? alert.cdm_id.slice(0, 16) : "--"}</code></td>
                <td>${formatDate(alert.tca)}</td>
                <td class="${getRiskClass(alert.collision_probability)}">${alert.collision_probability ? formatProbability(alert.collision_probability) : "--"}</td>
                <td><span class="status-badge ${alert.acknowledged ? "connected" : "pending"}">${alert.acknowledged ? "Acked" : "New"}</span></td>
            </tr>
        `;
    })
    .join("");
}

function updateMetrics(metrics) {
  if (!metrics) return;
  state.metrics = metrics;

  elements.metricAnnounced.textContent = metrics.cdms_announced || 0;
  elements.metricWithdrawn.textContent = metrics.cdms_withdrawn || 0;
  elements.metricSent.textContent = metrics.messages_sent || 0;
  elements.metricErrors.textContent = metrics.errors || 0;
}

function updateTopology() {
  const svg = elements.topologySvg;
  const width = 600;
  const height = 300;
  const centerX = width / 2;
  const centerY = height / 2;

  // Clear existing
  svg.innerHTML = "";

  // Create self node at center
  const selfNode = createNode(centerX, centerY, "This Node", true);

  // Create peer nodes in a circle
  const peerNodes = [];
  const radius = 100;
  state.peers.forEach((peer, i) => {
    const angle = (2 * Math.PI * i) / Math.max(state.peers.length, 1);
    const x = centerX + radius * Math.cos(angle - Math.PI / 2);
    const y = centerY + radius * Math.sin(angle - Math.PI / 2);

    // Create edge first (so it's behind nodes)
    const edge = document.createElementNS("http://www.w3.org/2000/svg", "line");
    edge.setAttribute("x1", centerX);
    edge.setAttribute("y1", centerY);
    edge.setAttribute("x2", x);
    edge.setAttribute("y2", y);
    edge.setAttribute(
      "class",
      `topology-edge ${peer.status === "Connected" || peer.status === "connected" ? "active" : ""}`,
    );
    svg.appendChild(edge);

    // Create peer node
    const label = peer.id ? peer.id.slice(0, 6) : `Peer ${i + 1}`;
    const node = createNode(x, y, label, false);
    peerNodes.push(node);
  });

  // Add self node last so it's on top
  svg.appendChild(selfNode);
  peerNodes.forEach((node) => svg.appendChild(node));
}

function createNode(x, y, label, isSelf) {
  const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
  g.setAttribute("class", "topology-node");
  g.setAttribute("transform", `translate(${x}, ${y})`);

  const circle = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "circle",
  );
  circle.setAttribute("r", isSelf ? 25 : 20);
  circle.setAttribute("class", `node-circle ${isSelf ? "self" : ""}`);

  const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
  text.setAttribute("y", isSelf ? 35 : 30);
  text.setAttribute("class", "node-label");
  text.textContent = label;

  g.appendChild(circle);
  g.appendChild(text);

  return g;
}

// Utility Functions
function formatUptime(seconds) {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
  if (seconds < 86400)
    return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
  return `${Math.floor(seconds / 86400)}d ${Math.floor((seconds % 86400) / 3600)}h`;
}

function formatDate(dateStr) {
  if (!dateStr) return "--";
  try {
    const date = new Date(dateStr);
    return (
      date.toLocaleDateString() +
      " " +
      date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    );
  } catch {
    return dateStr;
  }
}

function formatProbability(prob) {
  if (prob >= 0.01) return (prob * 100).toFixed(1) + "%";
  return prob.toExponential(2);
}

function getStatusClass(status) {
  if (!status) return "";
  const s = status.toLowerCase();
  if (s === "connected" || s === "active") return "connected";
  if (s === "pending" || s === "connecting") return "pending";
  return "disconnected";
}

function getRiskClass(probability) {
  if (!probability) return "";
  if (probability >= 1e-4) return "risk-high";
  if (probability >= 1e-6) return "risk-medium";
  return "risk-low";
}

// Modal toggle function (global for onclick)
window.toggleDemoInfo = function () {
  const modal = document.getElementById("demo-modal");
  if (modal) {
    modal.style.display = modal.style.display === "none" ? "flex" : "none";
  }
};

// Main Update Loop
async function refresh() {
  try {
    // Fetch all data in parallel (including alerts from Constellation Hub)
    const [health, peers, cdms, metrics, alerts] = await Promise.all([
      fetchHealth().catch(() => null),
      fetchPeers().catch(() => ({ peers: [] })),
      fetchCdms().catch(() => ({ cdms: [] })),
      fetchMetrics(),
      fetchAlerts(),
    ]);

    if (health) {
      updateConnectionStatus(true);
      updateHealth(health);
    } else {
      throw new Error("No health data");
    }

    updatePeers(peers);
    updateCdms(cdms);
    updateMetrics(metrics);
    updateAlerts(alerts);
    updateTopology();
  } catch (error) {
    console.error("Refresh failed:", error);
    updateConnectionStatus(false);
    state.retryCount++;
  }
}

// Initialize
function init() {
  console.log("SpaceComms Dashboard initializing...");
  console.log("🔬 Demo Mode: Connecting to mock services");

  // Check for custom node URL in query params
  const urlParams = new URLSearchParams(window.location.search);
  const customUrl = urlParams.get("node");
  if (customUrl) {
    CONFIG.nodeUrl = customUrl;
    console.log("Using custom node URL:", CONFIG.nodeUrl);
  }

  const hubUrl = urlParams.get("hub");
  if (hubUrl) {
    CONFIG.constellationHubUrl = hubUrl;
    console.log("Using custom hub URL:", CONFIG.constellationHubUrl);
  }

  // Initial refresh
  refresh();

  // Set up periodic refresh
  setInterval(refresh, CONFIG.refreshInterval);

  console.log(
    "Dashboard ready. Refreshing every",
    CONFIG.refreshInterval / 1000,
    "seconds",
  );
}

// Start when DOM is ready
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", init);
} else {
  init();
}

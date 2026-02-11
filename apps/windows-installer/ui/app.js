const invoke = window.__TAURI__?.core?.invoke;
if (!invoke) throw new Error("Tauri API not available");

const tabs = document.querySelectorAll(".tab");
const panels = document.querySelectorAll(".panel");

function showPanel(id) {
  tabs.forEach((t) => t.classList.toggle("active", t.dataset.tab === id));
  panels.forEach((p) => p.classList.toggle("active", p.id === "panel-" + id));
}

tabs.forEach((t) => {
  t.addEventListener("click", () => showPanel(t.dataset.tab));
});

// --- Install ---
const nodeStatus = document.getElementById("node-status");
const openclawStatus = document.getElementById("openclaw-status");
const pathStatus = document.getElementById("path-status");
const btnCheckNode = document.getElementById("btn-check-node");
const btnInstallOpenclaw = document.getElementById("btn-install-openclaw");

async function refreshInstallStatus() {
  nodeStatus.textContent = "Checking…";
  openclawStatus.textContent = "—";
  pathStatus.textContent = "—";
  try {
    const node = await invoke("check_node");
    nodeStatus.textContent = node.message;
    if (node.ok) {
      const oc = await invoke("check_openclaw").catch(() => ({ found: false, message: "openclaw not found", version: null }));
      openclawStatus.textContent = oc.message || "—";
      if (oc.found) {
        const pathMsg = await invoke("ensure_path").catch((e) => e);
        pathStatus.textContent = typeof pathMsg === "string" ? pathMsg : pathMsg?.message || "—";
      } else {
        pathStatus.textContent = "Install OpenClaw first.";
      }
    }
  } catch (e) {
    nodeStatus.textContent = "Error: " + e;
  }
}

btnCheckNode.addEventListener("click", async () => {
  btnCheckNode.disabled = true;
  try {
    const node = await invoke("check_node");
    if (!node.ok && node.found === false) {
      const msg = await invoke("install_node").catch((e) => e);
      alert(typeof msg === "string" ? msg : msg?.message || String(msg));
    }
    await refreshInstallStatus();
  } catch (e) {
    alert("Error: " + e);
  } finally {
    btnCheckNode.disabled = false;
  }
});

btnInstallOpenclaw.addEventListener("click", async () => {
  btnInstallOpenclaw.disabled = true;
  try {
    const msg = await invoke("install_openclaw");
    alert(msg);
    await refreshInstallStatus();
  } catch (e) {
    alert("Error: " + e);
  } finally {
    btnInstallOpenclaw.disabled = false;
  }
});

// --- Config ---
const configPathEl = document.getElementById("config-path");
const configForm = document.getElementById("config-form");
const gatewayMode = document.getElementById("gateway-mode");
const gatewayPort = document.getElementById("gateway-port");
const webApiKey = document.getElementById("web-api-key");
const btnLoadConfig = document.getElementById("btn-load-config");

async function loadConfigPath() {
  try {
    const path = await invoke("get_config_path");
    configPathEl.textContent = path;
  } catch {
    configPathEl.textContent = "(could not resolve)";
  }
}

btnLoadConfig.addEventListener("click", async () => {
  try {
    const port = await invoke("get_config_value", { path: "gateway.port" }).catch(() => "18789");
    const mode = await invoke("get_config_value", { path: "gateway.mode" }).catch(() => "local");
    gatewayPort.value = port || "18789";
    gatewayMode.value = mode || "local";
    const apiKey = await invoke("get_config_value", { path: "tools.web.search.apiKey" }).catch(() => "");
    webApiKey.value = apiKey || "";
  } catch (e) {
    alert("Load config: " + e);
  }
});

configForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  try {
    await invoke("set_config_value", { path: "gateway.port", value: gatewayPort.value });
    await invoke("set_config_value", { path: "gateway.mode", value: gatewayMode.value });
    if (webApiKey.value.trim()) {
      await invoke("set_config_value", { path: "tools.web.search.apiKey", value: webApiKey.value });
    }
    alert("Saved.");
  } catch (err) {
    alert("Save failed: " + err);
  }
});

// --- Security ---
const scoreValue = document.getElementById("score-value");
const scoreLabel = document.getElementById("score-label");
const scoreDetails = document.getElementById("score-details");
const scoreCircle = document.getElementById("score-circle");
const findingsList = document.getElementById("security-findings");
const btnSecurityCheck = document.getElementById("btn-security-check");
const btnSecurityDeep = document.getElementById("btn-security-deep");
const btnSecurityFix = document.getElementById("btn-security-fix");

function renderScore(result) {
  scoreValue.textContent = result.score;
  scoreLabel.textContent = result.label;
  scoreDetails.textContent = result.message;
  scoreCircle.classList.remove("good", "warn", "bad");
  if (result.score >= 80) scoreCircle.classList.add("good");
  else if (result.score >= 50) scoreCircle.classList.add("warn");
  else scoreCircle.classList.add("bad");
  findingsList.innerHTML = "";
  if (result.findings && result.findings.length > 0) {
    const ul = document.createElement("ul");
    result.findings.forEach((f) => {
      const li = document.createElement("li");
      li.className = f.severity;
      li.textContent = `[${f.severity}] ${f.title}: ${f.detail}`;
      ul.appendChild(li);
    });
    findingsList.appendChild(ul);
  }
}

btnSecurityCheck.addEventListener("click", async () => {
  btnSecurityCheck.disabled = true;
  try {
    const result = await invoke("run_security_audit", { deep: false });
    renderScore(result);
  } catch (e) {
    scoreValue.textContent = "—";
    scoreLabel.textContent = "Error";
    scoreDetails.textContent = String(e);
  } finally {
    btnSecurityCheck.disabled = false;
  }
});

btnSecurityDeep.addEventListener("click", async () => {
  btnSecurityDeep.disabled = true;
  try {
    const result = await invoke("run_security_audit", { deep: true });
    renderScore(result);
  } catch (e) {
    scoreValue.textContent = "—";
    scoreLabel.textContent = "Error";
    scoreDetails.textContent = String(e);
  } finally {
    btnSecurityDeep.disabled = false;
  }
});

btnSecurityFix.addEventListener("click", async () => {
  btnSecurityFix.disabled = true;
  try {
    const msg = await invoke("run_security_fix");
    alert(msg);
    const result = await invoke("run_security_audit", { deep: false });
    renderScore(result);
  } catch (e) {
    alert("Fix failed: " + e);
  } finally {
    btnSecurityFix.disabled = false;
  }
});

// Init
loadConfigPath();
refreshInstallStatus();

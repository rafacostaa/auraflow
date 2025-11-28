const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

const startBtn = document.getElementById('startBtn');
const stopBtn = document.getElementById('stopBtn');
const statusIndicator = document.getElementById('statusIndicator');
const statusText = document.getElementById('statusText');
const logDiv = document.getElementById('log');
const idleThreshold = document.getElementById('idleThreshold');
const jiggleInterval = document.getElementById('jiggleInterval');
const saveSettings = document.getElementById('saveSettings');

let isRunning = false;

// Add log entry
function addLog(message) {
    const entry = document.createElement('div');
    entry.className = 'log-entry';
    
    const time = new Date().toLocaleTimeString();
    entry.innerHTML = `<span class="log-time">${time}</span>${message}`;
    
    logDiv.insertBefore(entry, logDiv.firstChild);
    
    // Keep only last 50 entries
    while (logDiv.children.length > 50) {
        logDiv.removeChild(logDiv.lastChild);
    }
}

// Update UI state
function updateUI(running) {
    isRunning = running;
    startBtn.disabled = running;
    stopBtn.disabled = !running;
    
    if (running) {
        statusIndicator.classList.add('active');
        statusText.textContent = 'Monitoring for idle activity...';
    } else {
        statusIndicator.classList.remove('active');
        statusText.textContent = 'Ready to start';
    }
}

// Start jiggler
startBtn.addEventListener('click', async () => {
    try {
        const result = await invoke('start_jiggler');
        addLog(result);
        updateUI(true);
    } catch (error) {
        addLog(`Error: ${error}`);
    }
});

// Stop jiggler
stopBtn.addEventListener('click', async () => {
    try {
        const result = await invoke('stop_jiggler');
        addLog(result);
        updateUI(false);
    } catch (error) {
        addLog(`Error: ${error}`);
    }
});

// Save settings
saveSettings.addEventListener('click', async () => {
    try {
        const result = await invoke('update_settings', {
            idleThreshold: parseInt(idleThreshold.value),
            jiggleInterval: parseInt(jiggleInterval.value)
        });
        addLog(result);
    } catch (error) {
        addLog(`Error: ${error}`);
    }
});

// Listen to events from Rust
listen('status', (event) => {
    statusText.textContent = event.payload;
    addLog(event.payload);
});

listen('jiggle', (event) => {
    addLog(event.payload);
});

// Load initial settings
async function loadSettings() {
    try {
        const [idle, interval] = await invoke('get_settings');
        idleThreshold.value = idle;
        jiggleInterval.value = interval;
    } catch (error) {
        console.error('Failed to load settings:', error);
    }
}

loadSettings();

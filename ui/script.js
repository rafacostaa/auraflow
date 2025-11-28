// Simple direct approach - wait for everything to load
window.addEventListener('DOMContentLoaded', async () => {
    console.log('Loading...');
    
    // Wait a bit for Tauri to inject its APIs
    await new Promise(resolve => setTimeout(resolve, 100));
    
    // Find ALL window properties with TAURI in the name
    const tauriKeys = Object.keys(window).filter(k => k.includes('TAURI'));
    console.log('Tauri keys found:', tauriKeys);
    
    // Get the invoke function - try all possibilities
    const invoke = window.__TAURI_INVOKE__ || 
                   (window.__TAURI__ && window.__TAURI__.invoke) ||
                   (window.__TAURI__ && window.__TAURI__.tauri && window.__TAURI__.tauri.invoke);
    
    if (!invoke) {
        document.body.innerHTML = '<h1 style="color:red;padding:20px;">ERROR: Tauri invoke not found!<br><br>Found: ' + tauriKeys.join(', ') + '</h1>';
        return;
    }
    
    console.log('Invoke function loaded!');
    
    // Get DOM elements
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

    // Start button
    startBtn.onclick = async () => {
        addLog('Starting jiggler...');
        try {
            const result = await invoke('start_jiggler');
            addLog(result);
            updateUI(true);
        } catch (error) {
            addLog(`Error: ${error}`);
        }
    };

    // Stop button
    stopBtn.onclick = async () => {
        addLog('Stopping jiggler...');
        try {
            const result = await invoke('stop_jiggler');
            addLog(result);
            updateUI(false);
        } catch (error) {
            addLog(`Error: ${error}`);
        }
    };

    // Save settings button
    saveSettings.onclick = async () => {
        try {
            const result = await invoke('update_settings', {
                idleThreshold: parseInt(idleThreshold.value),
                jiggleInterval: parseInt(jiggleInterval.value)
            });
            addLog(result);
        } catch (error) {
            addLog(`Error: ${error}`);
        }
    };

    // Load initial settings
    try {
        const [idle, interval] = await invoke('get_settings');
        idleThreshold.value = idle;
        jiggleInterval.value = interval;
        addLog('Settings loaded successfully');
    } catch (error) {
        console.error('Failed to load settings:', error);
        addLog('Using default settings');
    }
    
    addLog('AuraFlow ready!');
});

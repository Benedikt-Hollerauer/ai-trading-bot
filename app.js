const stocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corporation' },
    { symbol: 'AMZN', name: 'Amazon.com Inc.' },
    { symbol: 'TSLA', name: 'Tesla, Inc.' }
];

let currentPrice = 0;
let autoRefreshInterval = null;  // Store the interval ID
let countdownInterval = null;    // Store the countdown interval ID
let nextRefreshTime = null;      // Store the next refresh timestamp

function initializeStockSelector() {
    const stockSelect = document.getElementById('stockSelect');
    stockSelect.innerHTML = stocks
        .map(stock => `<option value="${stock.symbol}">${stock.symbol} - ${stock.name}</option>`)
        .join('');
    
    // Set initial stock name
    const initialStock = getSelectedStock();
    document.getElementById('stockName').textContent = initialStock.name;
}

function getFormattedTimestamp() {
    const now = new Date();
    return now.toLocaleDateString() + " " + now.toLocaleTimeString();
}

function updateOutput(message) {
    const outputDiv = document.getElementById('output');
    const timestamp = getFormattedTimestamp();
    outputDiv.textContent = `[${timestamp}] ${message}`;
    
    // Update countdown if auto-refresh is active
    if (autoRefreshInterval) {
        updateCountdown();
    }
}

function analyzeInvestment() {
    const amount = document.getElementById('amountInput').value;
    const ticker = document.getElementById('stockSelect').value;
    updateOutput(`Analyzing ${ticker} with €${amount}...`);

    fetch('/analyze', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ticker, amount: parseFloat(amount) })
    })
        .then(response => {
            if (!response.ok) {
                return response.text().then(text => { throw new Error(text); });
            }
            return response.json();
        })
        .then(data => {
            if (data.error_type) {
                // Handle error response
                updateOutput(`[${data.error_type}] ${data.message}: ${data.details || ''}`);
            } else {
                // Handle successful analysis response
                const quantity = Number(data.quantity) || 0;
                const price = Number(data.price) || 0;
                currentPrice = price;
                updateOutput(`${data.message} - Order Type: ${data.order_type}`);
                updateStockInfo(getSelectedStock(), amount);
            }
        })
        .catch(error => {
            updateOutput(`Error: ${error.message}`);
        });
    
    return false; // Prevent form submission
}

// Update stock name immediately on ticker change.
document.getElementById('stockSelect').addEventListener('change', () => {
    const stock = getSelectedStock();
    document.getElementById('stockName').textContent = stock.name;
});

function getSelectedStock() {
    const ticker = document.getElementById('stockSelect').value;
    return stocks.find(s => s.symbol === ticker) || { symbol: ticker, name: ticker };
}

function updateStockInfo(stock, amount) {
    const amt = Number(amount) || 0;
    document.getElementById('stockName').textContent = stock.name;
    document.getElementById('investedAmount').textContent = `€${amt.toFixed(2)}`;
    document.getElementById('currentPrice').textContent = `€${Number(currentPrice).toFixed(2)}`;
}

function updateCountdown() {
    if (!nextRefreshTime) return;
    
    const now = new Date().getTime();
    const timeLeft = nextRefreshTime - now;
    
    if (timeLeft <= 0) {
        return;  // The refresh function will handle the reset
    }

    // Calculate minutes and seconds
    const minutes = Math.floor(timeLeft / (1000 * 60));
    const seconds = Math.floor((timeLeft % (1000 * 60)) / 1000);
    
    // Get the current output text, removing any existing countdown
    const outputDiv = document.getElementById('output');
    let currentText = outputDiv.textContent;
    currentText = currentText.replace(/\s*\(Next refresh in:.*\)$/, '');
    
    // Append the countdown
    outputDiv.textContent = `${currentText} (Next refresh in: ${minutes}m ${seconds}s)`;
}

function refreshStockData() {
    const amount = document.getElementById('amountInput').value;
    const ticker = document.getElementById('stockSelect').value;
    updateOutput(`Analyzing ${ticker} with €${amount}...`);

    fetch('/refresh', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ticker })
    })
        .then(response => {
            if (!response.ok) {
                return response.text().then(text => { throw new Error(text); });
            }
            return response.json();
        })
        .then(data => {
            console.log(data);
            if (data.error_type) {
                // Handle error response
                updateOutput(`[${data.error_type}] ${data.message}: ${data.details || ''}`);
            } else {
                // Handle successful refresh response
                const invested = Number(data.invested_amount) || 0;
                const currPrice = Number(data.current_price) || 0;
                const stock = getSelectedStock();
                document.getElementById('stockName').textContent = stock.name;
                document.getElementById('investedAmount').textContent = `€${invested.toFixed(2)}`;
                document.getElementById('currentPrice').textContent = `€${currPrice.toFixed(2)}`;
                updateOutput(`Action taken: ${data.action_taken}`);
            }
        })
        .catch(error => {
            updateOutput(`Error: ${error.message}`);
        });
}

function startAutoRefresh(minutes) {
    // Clear any existing intervals
    stopAutoRefresh();
    
    // Convert minutes to milliseconds
    const interval = minutes * 60 * 1000;
    
    // Set next refresh time
    nextRefreshTime = new Date().getTime() + interval;
    
    // Set new intervals
    autoRefreshInterval = setInterval(() => {
        refreshStockData();
        nextRefreshTime = new Date().getTime() + interval;
    }, interval);
    
    // Start countdown update
    countdownInterval = setInterval(updateCountdown, 1000);
}

function stopAutoRefresh() {
    if (autoRefreshInterval) {
        clearInterval(autoRefreshInterval);
        clearInterval(countdownInterval);
        autoRefreshInterval = null;
        countdownInterval = null;
        nextRefreshTime = null;
    }
}

function toggleAutoRefresh() {
    const minutes = parseInt(document.getElementById('refreshInterval').value) || 5;
    const toggleButton = document.getElementById('autoRefreshToggle');
    
    if (autoRefreshInterval) {
        stopAutoRefresh();
        toggleButton.innerHTML = '<span class="material-icons">play_arrow</span> Start Auto-Refresh';
        toggleButton.classList.remove('active');
    } else {
        startAutoRefresh(minutes);
        toggleButton.innerHTML = '<span class="material-icons">stop</span> Stop Auto-Refresh';
        toggleButton.classList.add('active');
    }
}

// Modify the last part to include initialization of auto-refresh controls
document.addEventListener('DOMContentLoaded', () => {
    initializeStockSelector();
    refreshStockData();
    
    // Stop auto-refresh when changing stocks
    document.getElementById('stockSelect').addEventListener('change', () => {
        stopAutoRefresh();
        const toggleButton = document.getElementById('autoRefreshToggle');
        toggleButton.innerHTML = '<span class="material-icons">play_arrow</span> Start Auto-Refresh';
        toggleButton.classList.remove('active');
    });
});
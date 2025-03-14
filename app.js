const stocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corporation' },
    { symbol: 'AMZN', name: 'Amazon.com Inc.' },
    { symbol: 'TSLA', name: 'Tesla, Inc.' }
];

let currentPrice = 0;

function initializeStockSelector() {
    const stockSelect = document.getElementById('stockSelect');
    stockSelect.innerHTML = stocks
        .map(stock => `<option value="${stock.symbol}">${stock.symbol} - ${stock.name}</option>`)
        .join('');
}

function analyzeInvestment() {
    const amount = document.getElementById('amountInput').value;
    const ticker = document.getElementById('stockSelect').value;
    const outputDiv = document.getElementById('output');
    outputDiv.textContent = `Analyzing ${ticker} with €${amount}...`;

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
                outputDiv.textContent = `[${data.error_type}] ${data.message}: ${data.details || ''}`;
            } else {
                // Handle successful analysis response
                const quantity = Number(data.quantity) || 0;
                const price = Number(data.price) || 0;
                currentPrice = price;
                outputDiv.textContent = `${data.message} - Order Type: ${data.order_type}`;
                updateStockInfo(getSelectedStock(), amount);
            }
        })
        .catch(error => {
            outputDiv.textContent = `Error: ${error.message}`;
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

function refreshStockData() {
    const ticker = document.getElementById('stockSelect').value;
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
                document.getElementById('output').textContent = `[${data.error_type}] ${data.message}: ${data.details || ''}`;
            } else {
                // Handle successful refresh response
                const invested = Number(data.invested_amount) || 0;
                const currPrice = Number(data.current_price) || 0;
                const stock = getSelectedStock();
                document.getElementById('stockName').textContent = stock.name;
                document.getElementById('investedAmount').textContent = `€${invested.toFixed(2)}`;
                document.getElementById('currentPrice').textContent = `€${currPrice.toFixed(2)}`;
                document.getElementById('output').textContent = `Action taken: ${data.action_taken}`;
            }
        })
        .catch(error => {
            document.getElementById('output').textContent = `Error: ${error.message}`;
        });
}

// Modify the last line to include initialization
document.addEventListener('DOMContentLoaded', () => {
    initializeStockSelector();
    refreshStockData();
});
const stocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corporation' },
    { symbol: 'AMZN', name: 'Amazon.com Inc.' }
];

let currentPrice = 0;

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
            const quantity = Number(data.quantity) || 0;
            const price = Number(data.price) || 0;
            currentPrice = price;
            outputDiv.textContent = `[${data.error_type}] ${data.message}: ${data.details}`;
            updateStockInfo(getSelectedStock(), amount);
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
            const invested = Number(data.invested_amount) || 0;
            const currPrice = Number(data.current_price) || 0;
            const stock = getSelectedStock();
            document.getElementById('stockName').textContent = stock.name;
            document.getElementById('investedAmount').textContent = `€${invested.toFixed(2)}`;
            document.getElementById('currentPrice').textContent = `€${currPrice.toFixed(2)}`;
            document.getElementById('output').textContent = `[${data.error_type}] ${data.message}: ${data.details}`;
        })
        .catch(error => {
            document.getElementById('output').textContent = `Error: ${error.message}`;
        });
}

// Initial load
refreshStockData();
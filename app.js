const stocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corporation' },
    { symbol: 'AMZN', name: 'Amazon.com Inc.' }
];

let currentPrice = 0;

document.getElementById('analysisForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const amount = document.getElementById('amountInput').value;
    const ticker = document.getElementById('stockSelect').value;
    const outputDiv = document.getElementById('output');
    outputDiv.textContent = `Analyzing ${ticker} with €${amount}...`;

    try {
        const response = await fetch('/analyze', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ ticker, amount: parseFloat(amount) })
        });
        const responseBody = await response.text();
        if (!response.ok) {
            try {
                const errorData = JSON.parse(responseBody);
                outputDiv.textContent = `Error [${errorData.error_type}]: ${errorData.message}`;
            } catch {
                outputDiv.textContent = `HTTP Error ${response.status}: ${responseBody}`;
            }
            return;
        }
        const data = JSON.parse(responseBody);
        const quantity = Number(data.quantity) || 0;
        const price = Number(data.price) || 0;
        currentPrice = price;
        outputDiv.innerHTML = `
      <div class="result-item">Order Type: <b>${data.order_type}</b></div>
      <div class="result-item">Quantity: ${quantity.toFixed(2)} shares</div>
      <div class="result-item">Price: €${price.toFixed(2)}</div>
      <div class="result-status">${data.message}</div>
    `;
        updateStockInfo(getSelectedStock(), amount);
    } catch (error) {
        document.getElementById('output').textContent = `Network Error: ${error.message}`;
    }
});

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
            const invested = Number(data.invested_amount) || 0;
            const currPrice = Number(data.current_price) || 0;
            const stock = getSelectedStock();
            document.getElementById('stockName').textContent = stock.name;
            document.getElementById('investedAmount').textContent = `€${invested.toFixed(2)}`;
            document.getElementById('currentPrice').textContent = `€${currPrice.toFixed(2)}`;
            document.getElementById('output').textContent = data.action_taken;
        })
        .catch(error => {
            document.getElementById('output').textContent = `Error: ${error.message}`;
        });
}

// Initial load
refreshStockData();
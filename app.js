// app.js
const stocks = [
    {symbol: 'AAPL', name: 'Apple Inc.'},
    {symbol: 'GOOGL', name: 'Alphabet Inc.'},
    {symbol: 'MSFT', name: 'Microsoft Corporation'},
    {symbol: 'AMZN', name: 'Amazon.com Inc.'}
];

let currentPrice = 0;

// app.js
document.getElementById('analysisForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const amount = document.getElementById('amountInput').value;
    const ticker = document.getElementById('stockSelect').value;
    const outputDiv = document.getElementById('output');

    outputDiv.textContent = `Analyzing ${ticker} with €${amount}...`;

    try {
        const response = await fetch('/analyze', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({
                ticker: ticker,
                amount: parseFloat(amount)
            })
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

        // Update output section
        outputDiv.innerHTML = `
            <div class="result-item">Order Type: <b>${data.order_type}</b></div>
            <div class="result-item">Quantity: ${data.quantity.toFixed(2)} shares</div>
            <div class="result-item">Price: €${data.price.toFixed(2)}</div>
            <div class="result-status">${data.message}</div>
        `;

        // Update current price display
        currentPrice = data.price;
        updateStockInfo(
            stocks.find(s => s.symbol === ticker),
            amount
        );

    } catch (error) {
        outputDiv.textContent = `Network Error: ${error.message}`;
    }
});

function updateStockInfo(stock, amount) {
    document.getElementById('stockName').textContent = stock.name;
    document.getElementById('investedAmount').textContent = `€${amount}`;
    document.getElementById('currentPrice').textContent = `€${currentPrice.toFixed(2)}`;
}

function refreshStockData() {
    // Simulate price update
    currentPrice = 150 + (Date.now() % 100);
    const selectedStock = stocks.find(s => s.symbol === document.getElementById('stockSelect').value);
    const amount = document.getElementById('amountInput').value;
    updateStockInfo(selectedStock, amount);
}

// Initial load
refreshStockData();
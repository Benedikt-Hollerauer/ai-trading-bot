// app.js
const stocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corporation' },
    { symbol: 'AMZN', name: 'Amazon.com Inc.' }
];

let currentPrice = 0;

document.getElementById('analysisForm').addEventListener('submit', (e) => {
    e.preventDefault();
    const amount = document.getElementById('amountInput').value;
    const selectedStock = stocks.find(s => s.symbol === document.getElementById('stockSelect').value);

    document.getElementById('output').textContent =
        `Analyzing ${selectedStock.name} with €${amount}...`;

    updateStockInfo(selectedStock, amount);
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
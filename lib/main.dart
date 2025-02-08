// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:flutter/material.dart';
// import 'package:my_app/src/rust/api/simple.dart';
// import 'package:my_app/src/rust/frb_generated.dart';
// 
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const MyApp());
// }
// 
// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
// 
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
//         body: Center(
//           child: Text(
//               'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
//         ),
//       ),
//     );
//   }
// }
// 

import 'package:flutter/material.dart';
import 'package:my_app/src/rust/api/simple.dart';
import 'package:my_app/src/rust/frb_generated.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark().copyWith(
        primaryColor: Colors.blueGrey[900],
        colorScheme: ColorScheme.dark(
          primary: Colors.cyan[300]!,
          secondary: Colors.cyanAccent[100]!,
        ),
        scaffoldBackgroundColor: Colors.blueGrey[900],
      ),
      home: const MainPage(),
    );
  }
}

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  final _formKey = GlobalKey<FormState>();
  final _amountController = TextEditingController();
  String _selectedTicker = 'AAPL';
  String _output = 'Enter details to analyze';
  double _currentPrice = 0.0;
  final List<Map<String, dynamic>> _stocks = [
    {'symbol': 'AAPL', 'name': 'Apple Inc.'},
    {'symbol': 'GOOGL', 'name': 'Alphabet Inc.'},
    {'symbol': 'MSFT', 'name': 'Microsoft Corporation'},
    {'symbol': 'AMZN', 'name': 'Amazon.com Inc.'},
  ];

  @override
  Widget build(BuildContext context) {
    final screenHeight = MediaQuery.of(context).size.height;
    final appBarHeight = screenHeight * 0.18;
    final bodyHeight = screenHeight * 0.82;

    return Scaffold(
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(appBarHeight),
        child: AppBar(
          toolbarHeight: appBarHeight,
          title: const Padding(
            padding: EdgeInsets.symmetric(vertical: 15),
            child: Text(
              'Stock Analyzer',
              style: TextStyle(fontSize: 24),
            ),
          ),
          actions: [
            IconButton(
              icon: const Icon(Icons.code, size: 28),
              onPressed: () => _launchURL('https://github.com/yourprofile'),
            ),
            IconButton(
              icon: const Icon(Icons.public, size: 28),
              onPressed: () => _launchURL('https://linkedin.com/in/yourprofile'),
            ),
          ],
        ),
      ),
      body: SizedBox(
        height: bodyHeight,
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: LayoutBuilder(
            builder: (context, constraints) {
              if (constraints.maxWidth > 800) {
                return _buildDesktopLayout();
              } else {
                return _buildMobileLayout();
              }
            },
          ),
        ),
      ),
    );
  }

  Widget _buildDesktopLayout() {
    return Column(
      children: [
        Expanded(
          flex: 5,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Expanded(child: _buildInputForm()),
              const SizedBox(width: 20),
              Expanded(child: _buildOutputSection()),
            ],
          ),
        ),
        const SizedBox(height: 20),
        Expanded(
          flex: 5,
          child: _buildInfoSection(),
        ),
      ],
    );
  }

  Widget _buildMobileLayout() {
    return SingleChildScrollView(
      child: Column(
        children: [
          SizedBox(
            height: MediaQuery.of(context).size.height * 0.4,
            child: _buildInputForm(),
          ),
          const SizedBox(height: 20),
          SizedBox(
            height: MediaQuery.of(context).size.height * 0.4,
            child: _buildOutputSection(),
          ),
          const SizedBox(height: 20),
          _buildInfoSection(),
        ],
      ),
    );
  }

  Widget _buildInputForm() {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 600),
      child: Card(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Form(
            key: _formKey,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                DropdownButtonFormField<String>(
                  value: _selectedTicker,
                  decoration: const InputDecoration(
                    labelText: 'Stock Ticker',
                    prefixIcon: Icon(Icons.trending_up),
                  ),
                  items: _stocks
                      .map((stock) => DropdownMenuItem<String>(
                            value: stock['symbol'] as String,
                            child: Text('${stock['symbol']} - ${stock['name']}'),
                          ))
                      .toList(),
                  onChanged: (value) => setState(() => _selectedTicker = value!),
                ),
                const SizedBox(height: 20),
                TextFormField(
                  controller: _amountController,
                  keyboardType: TextInputType.number,
                  decoration: const InputDecoration(
                    labelText: 'Investment Amount (€)',
                    prefixIcon: Icon(Icons.euro),
                  ),
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return 'Please enter an amount';
                    }
                    if (double.tryParse(value) == null) {
                      return 'Enter valid number';
                    }
                    return null;
                  },
                ),
                const SizedBox(height: 20),
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: _submitForm,
                    child: const Text('Analyze Investment'),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildOutputSection() {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 600),
      child: Card(
        color: Colors.blueGrey[900],
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: SingleChildScrollView(
            child: Text(
              _output,
              style: TextStyle(
                color: _output.startsWith('Error') 
                    ? Colors.redAccent[100]
                    : Colors.white,
                fontSize: 16,
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildInfoSection() {
    final currentStock = _stocks.firstWhere(
      (stock) => stock['symbol'] == _selectedTicker,
    );

    return Card(
      color: Colors.blueGrey[800],
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('Stock Name:', style: TextStyle(fontSize: 16)),
                Text(currentStock['name'], style: const TextStyle(fontSize: 16)),
              ],
            ),
            const SizedBox(height: 15),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('Invested Amount:', style: TextStyle(fontSize: 16)),
                Text(
                  '€${_amountController.text}',
                  style: const TextStyle(fontSize: 16),
                ),
              ],
            ),
            const SizedBox(height: 15),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text('Current Price:', style: TextStyle(fontSize: 16)),
                Text(
                  '€${_currentPrice.toStringAsFixed(2)}',
                  style: const TextStyle(fontSize: 16),
                ),
              ],
            ),
            const SizedBox(height: 20),
            ElevatedButton.icon(
              icon: const Icon(Icons.refresh),
              label: const Text('Refresh Data'),
              onPressed: _refreshStockData,
            ),
          ],
        ),
      ),
    );
  }

  void _submitForm() {
    if (_formKey.currentState!.validate()) {
      final currentStock = _stocks.firstWhere(
        (stock) => stock['symbol'] == _selectedTicker,
      );
      setState(() {
        _output = 'Analyzing ${currentStock['name']} '
            'with €${_amountController.text}...';
      });
      _refreshStockData();
    }
  }


  void _refreshStockData() {
    // Simulated price update
    setState(() {
      _currentPrice = 150.0 + (DateTime.now().second % 100);
    });
  }

  Future<void> _launchURL(String url) async {
    if (!await launchUrl(Uri.parse(url))) {
      throw Exception('Could not launch $url');
    }
  }
}

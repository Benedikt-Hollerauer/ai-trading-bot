import 'package:flutter/material.dart';
import 'package:my_app/src/rust/api/simple.dart';
import 'package:my_app/src/rust/frb_generated.dart';
import 'package:url_launcher/url_launcher.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData.dark().copyWith(
        primaryColor: Colors.blueGrey[900],
        scaffoldBackgroundColor: Colors.blueGrey[900],
        cardColor: Colors.black,
        cardTheme: const CardTheme(color: Colors.black),
        colorScheme: ColorScheme.dark(
          primary: Colors.cyan[300]!,
          secondary: Colors.cyanAccent[100]!,
        ),
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

  final ButtonStyle _commonButtonStyle = ElevatedButton.styleFrom(
    minimumSize: const Size(double.infinity, 60),
    textStyle: const TextStyle(fontSize: 20),
  );

  final TextStyle _infoTextStyle = const TextStyle(fontSize: 20);

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
          title: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 15),
            child: const Text('Stock Analyzer', style: TextStyle(fontSize: 24)),
          ),
          actions: [
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: IconButton(
                icon: const Icon(Icons.code, size: 28),
                onPressed: () => _launchURL('https://github.com/yourprofile'),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: IconButton(
                icon: const Icon(Icons.public, size: 28),
                onPressed: () =>
                    _launchURL('https://linkedin.com/in/yourprofile'),
              ),
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
              return constraints.maxWidth > 800
                  ? _buildDesktopLayout()
                  : _buildMobileLayout();
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
        Expanded(flex: 5, child: _buildInfoSection()),
      ],
    );
  }

  Widget _buildMobileLayout() {
    return SingleChildScrollView(
      child: Column(
        children: [
          SizedBox(
              height: MediaQuery.of(context).size.height * 0.4,
              child: _buildInputForm()),
          const SizedBox(height: 20),
          SizedBox(
              height: MediaQuery.of(context).size.height * 0.4,
              child: _buildOutputSection()),
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
                            child: Text(
                                '${stock['symbol']} - ${stock['name']}'),
                          ))
                      .toList(),
                  onChanged: (value) =>
                      setState(() => _selectedTicker = value!),
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
                  height: 60,
                  child: ElevatedButton(
                    style: _commonButtonStyle,
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
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: SingleChildScrollView(
            child: Text(
              _output,
              style: TextStyle(
                color: _output.startsWith('Error')
                    ? Colors.redAccent[100]
                    : Colors.white,
                fontSize: 20,
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildInfoSection() {
    final currentStock = _stocks
        .firstWhere((stock) => stock['symbol'] == _selectedTicker);
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Row(
          children: [
            Expanded(
              flex: 7,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _buildInfoRow('Stock Name:', currentStock['name']),
                  const SizedBox(height: 10),
                  _buildInfoRow('Invested Amount:', '€${_amountController.text}'),
                  const SizedBox(height: 10),
                  _buildInfoRow('Current Price:',
                      '€${_currentPrice.toStringAsFixed(2)}'),
                ],
              ),
            ),
            Expanded(
              flex: 3,
              child: Center(
                child: ElevatedButton.icon(
                  style: _commonButtonStyle,
                  icon: const Icon(Icons.refresh),
                  label: const Text('Refresh Data'),
                  onPressed: _refreshStockData,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInfoRow(String label, String value) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(label, style: _infoTextStyle),
        Text(value, style: _infoTextStyle),
      ],
    );
  }

  void _submitForm() {
    if (_formKey.currentState!.validate()) {
      final currentStock =
          _stocks.firstWhere((stock) => stock['symbol'] == _selectedTicker);
      setState(() {
        _output =
            'Analyzing ${currentStock['name']} with €${_amountController.text}...';
      });
      _refreshStockData();
    }
  }

  void _refreshStockData() {
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

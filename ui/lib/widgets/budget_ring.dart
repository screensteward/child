import 'package:flutter/material.dart';

class BudgetRing extends StatelessWidget {
  final int used;
  final int? budget;

  const BudgetRing({super.key, required this.used, this.budget});

  @override
  Widget build(BuildContext context) {
    final hasBudget = budget != null && budget! > 0;
    final ratio = hasBudget ? (used / budget!).clamp(0.0, 1.0) : 0.0;
    final remaining = hasBudget ? (budget! - used).clamp(0, budget!) : 0;

    return SizedBox(
      width: 240,
      height: 240,
      child: Stack(
        alignment: Alignment.center,
        children: [
          SizedBox.expand(
            child: CircularProgressIndicator(
              value: ratio,
              strokeWidth: 16,
              backgroundColor: Colors.grey[300],
            ),
          ),
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                hasBudget ? '$remaining min' : '—',
                style: Theme.of(context).textTheme.headlineLarge,
              ),
              Text('restant', style: Theme.of(context).textTheme.bodyMedium),
            ],
          ),
        ],
      ),
    );
  }
}

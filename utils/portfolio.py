from utils.order import Order
from utils.position import Position
import pandas as pd

class Portfolio:
    def __init__(self, initial_cash: float = 0) -> None:
        self.cash = initial_cash
        self.positions = {}
        self.portfolio_value = initial_cash

    def add_cash_flow(self, value: float) -> None:
        self.cash += value

    def order_filled(self, order: Order) -> None:
        # order_value could be positive(buy) and negative(sell)
        self.cash -= order.order_value
        if self.cash < 0:
            raise Exception("Negative cash error.")
        if order.symbol not in self.positions.keys():
            self.positions[order.symbol] = Position(order.symbol)
        self.positions[order.symbol].order_filled(order)
        
    def get_snapshot(self):
        return {"portfolio_value": self.portfolio_value, "positions": self.positions, "cash": self.cash}

    def update_portfolio(self, prices: dict):
        for symbol, price in prices.items():
            if symbol in self.positions.keys():
                self.positions[symbol].update_position(price)

        self.portfolio_value = sum([position.position_value for position in self.positions.values()])

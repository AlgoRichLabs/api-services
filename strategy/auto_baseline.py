import pandas as pd
import random
from datetime import datetime
from utils.portfolio import Portfolio
from utils.order import Order


MONTHLY_CASH_FLOW = 1000
class AutoBaseline:
    """
    Baseline model for automatic and regular investment strategy.
    """
    def __init__(self, initial_cash: float, start_date: str, end_date: str) -> None:
        self.portfolio = Portfolio(initial_cash)
        self.start_date = datetime.strptime(start_date, '%Y-%m-%d')
        self.end_date = datetime.strptime(end_date, '%Y-%m-%d')

    def read_history_data(self, path: str, symbol: str):
        df = pd.read_csv(path).sort_values(by="Date")
        df['Date'] = pd.to_datetime(df['Date'])
        self.df = df[(df['Date'] >= self.start_date) & (df['Date'] <= self.end_date)].set_index("Date")
        self.symbol = symbol

    def run(self):
        if self.df is None:
            raise ValueError("Historical data not loaded. Please run read_history_date() first.")
        portfolio_records = {}

        current_date = self.start_date
        current_month = None
        invested_this_month = False
        for current_date, record in self.df.iterrows():
            if current_date.month != current_month:
                current_month = current_date.month
                self.portfolio.add_cash_flow(MONTHLY_CASH_FLOW)
                invested_this_month = False
            
            if not invested_this_month:
                quantity = self.portfolio.cash // record["Close"]
                order = Order(self.symbol, "buy", record["Close"], quantity)
                self.portfolio.order_filled(order)
                invested_this_month = True

            self.portfolio.update_portfolio({self.symbol: record["Close"]})
            portfolio_records[current_date.strftime("%Y-%m-%d")] = self.portfolio.get_snapshot()

        return {date: info["portfolio_value"] + info["cash"] for date, info in portfolio_records.items()}
                



            










                

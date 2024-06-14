import pandas as pd
import random
from datetime import datetime
from utils.portfolio import Portfolio
from utils.order import Order


MONTHLY_CASH_FLOW = 1000
class AutoIndexRebalance:
    """
    Regularly investment fix amount of money into index funds. 
    Equally rebalance the portfolio periodially.
    """
    def __init__(self, initial_cash: float, start_date: str, end_date: str, rebalance_period: int=252) -> None:
        self.portfolio = Portfolio(initial_cash)
        self.start_date = datetime.strptime(start_date, '%Y-%m-%d')
        self.end_date = datetime.strptime(end_date, '%Y-%m-%d')
        self.index_num = 0
        self.dfs = {}
        self.rebalance_period = rebalance_period

    def read_history_data(self, path: str, symbol: str):
        df = pd.read_csv(path).sort_values(by="Date")
        df['Date'] = pd.to_datetime(df['Date'])
        self.dfs[symbol] = df[(df['Date'] >= self.start_date) & (df['Date'] <= self.end_date)].set_index("Date")
        self.index_num += 1

    def run(self):
        if len(self.dfs) == 0:
            raise ValueError("Historical data not loaded. Please run read_history_date() first.")
        portfolio_records = {}
        # Find common trading dates
        common_dates = set(self.dfs[next(iter(self.dfs))].index)
        for df in self.dfs.values():
            common_dates = common_dates.intersection(df.index)
        common_dates = sorted(common_dates)
        current_date = self.start_date
        current_month = None
        invested_this_month = False
        period_count = 0
        for current_date in common_dates:
            current_prices = {symbol: df.loc[current_date]['Close'] for symbol, df in self.dfs.items()}
            if current_date.month != current_month:
                current_month = current_date.month
                self.portfolio.add_cash_flow(MONTHLY_CASH_FLOW)
                invested_this_month = False
            
            if not invested_this_month:
                # current_prices = {symbol: df.loc[current_date]['Close'] for symbol, df in self.dfs.items()}
                invest_batch_value = self.portfolio.cash / self.index_num
                
                for symbol, price in current_prices.items():
                    order = Order(symbol, "buy", price, invest_batch_value // price)
                    self.portfolio.order_filled(order)

                self.portfolio.update_portfolio(current_prices)
                invested_this_month = True
            
            # print(period_count % 252)
            if period_count % 252 == 0:
                # if period_count == 1260:
                #     print("break")
                self.portfolio.update_portfolio(current_prices)
                total_value = self.portfolio.portfolio_value
                equal_value = total_value / len(current_prices)
                

                buy_orders = []
                sell_orders = []
                for symbol, price in current_prices.items():
                    desired_amount = equal_value // price
                    current_amount = self.portfolio.positions[symbol].amount if symbol in self.portfolio.positions else 0
                    # Sell 1 more quantity and buy 1 less quantity to avoid cash not enough
                    if current_amount >= desired_amount:
                        order = Order(symbol, "sell", price, current_amount - desired_amount + 1)
                        sell_orders.append(order)
                    else:
                        order = Order(symbol, "buy", price, desired_amount - current_amount - 1)
                        buy_orders.append(order)

                for order in sell_orders:
                    self.portfolio.order_filled(order)
                for order in buy_orders:
                    self.portfolio.order_filled(order)
                
                self.portfolio.update_portfolio(current_prices)
            
            period_count += 1
            self.portfolio.update_portfolio(current_prices)
            portfolio_records[current_date.strftime("%Y-%m-%d")] = self.portfolio.get_snapshot()
        
        return {date: info["portfolio_value"] + info["cash"] for date, info in portfolio_records.items()}


                



            










                

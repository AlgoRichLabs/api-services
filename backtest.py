from utils.metrics import Metrics
from strategy.auto_baseline import AutoBaseline
from strategy.auto_idx_rebalance import AutoIndexRebalance


# strategy = AutoBaseline(0, "2000-01-01", "2024-05-01")
strategy = AutoIndexRebalance(0, "2010-05-01", "2024-05-01", 252)
symbol = "SPY"
strategy.read_history_data(f"data/{symbol}/{symbol}.csv", symbol)
symbol = "IWY"
strategy.read_history_data(f"data/{symbol}/{symbol}.csv", symbol)

result = strategy.run()
    


    
    








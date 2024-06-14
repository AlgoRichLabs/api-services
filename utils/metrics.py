from typing import Union, Iterable
import numpy as np
import pandas as pd

class Metrics:
    @staticmethod
    def sharpe(prices: Union[Iterable[float], np.ndarray]) -> float:
        prices = pd.Series(prices)
        returns = prices.pct_change().dropna()
        return_avg = returns.mean()
        return_std = returns.std()
        
        return round(return_avg / return_std, 4)

    # Input should be daily close prices
    @staticmethod
    def annualized_return(prices: Union[Iterable[float], np.ndarray]) -> float:
        num_of_years = len(prices) / 252
        print(f"Number of years: {num_of_years}.")
        total_return = prices[-1] / prices[0]
        print(f"Total return is {total_return}.")

        return total_return ** (1 / num_of_years) - 1
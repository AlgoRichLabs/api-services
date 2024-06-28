from abc import ABC, abstractmethod
from typing import Dict, List

from utils.constants import *


class BaseExchange(ABC):
    def __init__(self, cfgs: Dict) -> None:
        self.cfgs = cfgs
        self.api_key = cfgs["api_key"]
        self.secret_key = cfgs["secret_key"]

    @abstractmethod
    def get_ticker(self, symbol: str) -> Dict:
        """
        param symbol: symbol to get ticker.
        return: the ticker information.
        """
        pass

    @abstractmethod
    def get_boo_price(self, symbol: str, side: SIDE.value) -> float:
        """
        param symbol: symbol to get price.
        param side: which side of the price to get.
        return: the best bid offer price.
        """
        pass

    def fetch_positions(self) -> List[Dict]:
        pass

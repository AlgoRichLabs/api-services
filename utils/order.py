class Order:
    def __init__(self, symbol, side, filled_price, quantity) -> None:
        self.symbol = symbol
        self.side = side
        self.filled_price = filled_price
        self.quantity = quantity
        if self.side == "buy":
            self.order_value = filled_price * quantity
        elif self.side == "sell":
            self.order_value = -filled_price * quantity
        else:
            raise Exception(f"Invalid side: {self.side}")

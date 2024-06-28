from typing import Dict, List
import requests
from datetime import datetime
import json
import hmac
import hashlib
import base64

from base import BaseExchange
from utils.constants import *


class OkxExchange(BaseExchange):
    def __init__(self, cfgs: Dict) -> None:
        super().__init__(cfgs)
        self.exchange_name = EXCHANGE.OKX.value
        self.passphrase = cfgs["passphrase"]
        self.session = requests.Session()
        self.session.headers.update({"X-MBX-APIKEY": self.api_key})
        self.base_url = "https://www.okx.com/v5/"

    def _get_request(self, data: Dict, endpoint: str) -> Dict:
        return self._send_request("GET", data, endpoint)

    def _post_request(self, data: Dict, endpoint: str) -> Dict:
        return self._send_request("POST", data, endpoint)

    def _delete_request(self, data: Dict, endpoint: str) -> Dict:
        return self._send_request("DELETE", data, endpoint)

    def _generate_signature(self, method: str, endpoint: str, query_string: str, body: str = "") -> (str, str):
        timestamp = datetime.utcnow().isoformat("T", "milliseconds") + "Z"
        pre_hash = f"{timestamp}{method}{endpoint}{query_string}{body}"
        signature = hmac.new(
            bytes(self.secret_key, "utf-8"),
            bytes(pre_hash, "utf-8"),
            hashlib.sha256
        ).digest()
        encoded_signature = base64.b64encode(signature)
        return encoded_signature, timestamp

    def _send_request(self, method: str, data: Dict, endpoint: str) -> Dict:
        query_string = "?" + "&".join([f"{d}={data[d]}" for d in data]) if data else ""
        body = json.dumps(data) if method == "POST" else ""
        encoded_signature, timestamp = self._generate_signature(method, endpoint, query_string, body)
        headers = self._get_header(encoded_signature, timestamp)

        url = self.base_url + endpoint + query_string
        if method == "GET":
            response = self.session.get(url, headers=headers)
        elif method == "POST":
            response = self.session.post(url, data=body, headers=headers)
        elif method == "DELETE":
            response = self.session.delete(url, headers=headers)
        else:
            raise ValueError(f"Invalid method: {method}.")

        results = response.json()
        return results.get("data", {})

    def _get_header(self, signature: bytes, timestamp: str) -> Dict:
        header = {"Content-Type": "application/json",
                  "OK-ACCESS-KEY": self.api_key,
                  "OK-ACCESS-SIGN": signature,
                  "OK-ACCESS-TIMESTAMP": timestamp,
                  "OK-ACCESS-PASSPHRASE": self.passphrase
                  }
        if self.cfgs.get("is_demo", False):
            header["x-simulated-trading"] = "1"
        return header

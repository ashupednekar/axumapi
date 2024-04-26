from typing import List, Dict

data = ["taylor swift", "ed sheeran"]


class Artists:

    def list(self) -> List[Dict]:
        return [{"id": i + 1, "name": x} for i, x in enumerate(data)]

    def retrieve(self, id: int) -> Dict:
        return {"id": data[id + 1]}

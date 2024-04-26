from typing import List, Dict

data = ["taylor swift", "ed sheeran"]

# TODO: handle classes
# class Artists:
#
#    def list(self) -> List[Dict]:
#        return [{"id": i + 1, "name": x} for i, x in enumerate(data)]
#
#    def retrieve(self, id: int) -> Dict:
#        return {"id": data[id + 1]}


def list() -> List[Dict]:
    return [{"id": i + 1, "name": x} for i, x in enumerate(data)]


def retrieve(id: int) -> Dict:
    if id < 1:
        raise IndexError("id should start from 1")
    return {"id": id, "name": data[id - 1]}

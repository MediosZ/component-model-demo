from word_filter import exports

class Filter(exports.Filter):
    def filter(self, x: str) -> str:
        if "Hello" in x:
            return "Hello from Python"
        else:
            return "# Unkown message"
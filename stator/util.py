import re


IP_ADDRESS_RE = re.compile(
  r"^([1-9]\d{0,2}|0)\.([1-9]\d{0,2}|0)\.([1-9]\d{0,2}|0)\.([1-9]\d{0,2}|0)$")


def convert_ip(host):
    match = IP_ADDRESS_RE.match(host)
    if not match:
        raise ValueError(
            "Ip address string required, got {!r}".format(host))
    parts = list(map(int, match.groups()))
    if any(p > 255 for p in parts):
        raise ValueError(
            "Ip address string required, got {!r}".format(host))
    return (parts[0] << 24) | (parts[1] << 16) | (parts[2] << 8) | parts[3]

import codecs
import json

from consts import MUN_AML


def discard_non_aml_municipalities():
    geojson = json.load(codecs.open('concelhos.geojson', 'r', 'utf-8-sig'))

    geojson['features'] = [feature for feature in geojson['features'] if feature['properties']['Concelho'] in MUN_AML]

    with open('aml.geojson', 'w') as f:
        json.dump(geojson, f)


def get_timestamp(string):
    if '1900' in string:
        # Let's call this midnight 2.0
        # (can be distinguished because the other midnight is 0, not 1440)
        return 24*60

    try:
        hour, minute = string.split(':')
    except ValueError:
        if len(string.strip()) > 0:
            raise Exception(f"Huh? Received a {string}. WTF CMet?!?")
        return None

    return int(hour) * 60 + int(minute)


def transpose(M):
    return [[M[j][i] for j in range(len(M))] for i in range(len(M[0]))]

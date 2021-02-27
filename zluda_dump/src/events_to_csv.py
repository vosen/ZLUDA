# Convert event trace json to csv
import json
import csv
import sys

def main(p):
    with open(p, 'rb') as f:
        event_text = f.read()
    try:
        event_trace = json.loads(event_text)
    except json.JSONDecodeError:
        event_text = bytearray(event_text)
        event_text.append(ord(']'))
        event_trace = json.loads(event_text)
    with open(f'{p}.csv', 'w', newline='') as csvfile:
        writer = csv.writer(csvfile, dialect='excel')
        writer.writerow(['name', 'cat', 'ts', 'dur', 'pid', 'tid'])
        for e in event_trace:
            if e['ph'] != 'X':
                continue
            writer.writerow([e['name'], e['cat'], e['ts'], e['dur'], e['pid'], e['tid']])

if __name__ == "__main__":
    main(sys.argv[1])

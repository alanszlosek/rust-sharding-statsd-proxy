# Used to generate messages.txt for testing
# python3 generate-messages.py > ../messages.txt
import random, string

def randomword(length):
   letters = string.ascii_lowercase
   return ''.join(random.choice(letters) for i in range(length))

def generate_measurement():
    metric = []
    tags = []
    num_parts = random.randint(1, 4)
    num_tags = random.randint(0,6)
    value_type = "2|c"

    i = 0
    while i < num_parts:
        word_len = random.randint(6,15)
        metric.append( randomword(word_len) )
        i += 1
    i = 0
    while i < num_tags:
        word_len1 = random.randint(6,15)
        word_len2 = random.randint(6,15)
        tags.append(
            "%s=%s" % (
                randomword(word_len1),
                randomword(word_len2)
            )
        )
        i += 1
    
    measurement = ".".join(metric)
    if num_tags > 0:
        measurement += "," + ",".join(tags)
    measurement += ":" + value_type
    return measurement

with open("messages.txt", "w") as f:
    i = 0
    while i < 1000:
        f.write( generate_measurement() + "\n")
        i += 1

    
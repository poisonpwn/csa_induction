from bs4 import BeautifulSoup
import sys
import requests


contest_id, problem_letter = sys.argv[1:3]
url = f"https://codeforces.com/contest/{contest_id}/problem/{problem_letter}"
html = requests.get(url).text

soup = BeautifulSoup(html, features="html.parser")

input_lines = soup.css.select(".input > pre")[0]
input_sequence = '\n'.join(["---INPUT---", *[i.string for i in input_lines]])

output_lines = soup.css.select(".output > pre")[0]
output_sequence = '\n'.join(["---OUTPUT---", *[i.string for i in output_lines]])

print(input_sequence, output_sequence,sep="\n")






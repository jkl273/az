#!/usr/bin/env python
# -*- coding: utf-8 -*-

import json, requests, random, re, sys, argparse

punct = u"。"
url = "http://www.aozorahack.net/api/v0.1/"
MAXLEN = 1000

def getbook(bid):
  cont = requests.get(url+"books/"+str(bid)+"/content")
  return cont.text

# 0: head
# 1: post-head
# 2: comment
# 3: post-comment
# 4: body

#             0   1   2   3
# -------------------------
# empty line  1   1   2   3
# "------"    0   2   3   4
# any other   0   4   2   4
def next(state, line):
  if state == 0: # head
    if line.strip() == '':
      return 1
    else:
      if len(line) > MAXLEN:
        sys.exit("line too long %s" % len(line))
      return 0
  if state == 1: # post-head
    if line.strip() == '':
      return 1
    elif re.match("^--*$", line.strip()):
      return 2
    else:
      return 4
  if state == 2: # comment
    if re.match("^--*$", line.strip()):
      return 3
    else:
      return 2
  if state == 3: # post-comment
    if line.strip() == '':
      return 3
    return 4
  if state == 4: # body
    return 4

def summary(text, code, num000):
  state = 0
  lines = 0
  num = num000
  for line in text.split("\r\n"):
    lineout = line.encode(code)
    state = next(state, line)
    dbgprint(("next state", state))
    if state == 0: # head
      print(lineout)
    elif state == 1: # post-head
      dbgprint((":1:", line))
      print("=======")
    elif state == 2: # comment
      dbgprint((":2:", line))
      pass
    elif state == 3: # post-comment
      dbgprint((":3:", line))
      pass
    elif state ==4: #body
      num = contline(line.strip("\r\n"), code, num)
      if num > 0:
        continue
      else:
        break

def contline(line, code, num):
  ret = num
  tt = re.search(punct, line)
  dbgprint((ret, tt, line))
  if line.strip() == '':
    print("") # empty line
    ret = ret
  elif tt == None:
    print(line.encode(code))
    ret = ret - 1
  else:
    ss = re.split(punct, line)
    dbgprint((":ss", ss))
    for i in ss:
      sys.stdout.write(i.encode(code))
      if i != '':
        sys.stdout.write(punct.encode(code))
        ret = ret -1
      if ret == 0: break
  dbgprint((":contline:ret:", ret))
  return ret

def dbgprint(x):
  if ppp.debug:
    print(x)
  

if __name__ == "__main__":
  parser = argparse.ArgumentParser(description='az')
  parser.add_argument('-d', '--debug', action='store_true', help='debug')
  parser.add_argument('-m', '--max', type=int, default=13772, help='max entries')
  parser.add_argument('-cm', '--checkmax', action='store_true', help='check max entries')
  parser.add_argument('-f', '--full', action='store_true', help='retrieve full text')
  parser.add_argument('-c', '--code', default='utf-8', help='out encoding')
  parser.add_argument('-n', '--num', type=int, default=1, help='number of lines')
  parser.add_argument('bookid', nargs='?', default=None)

  ppp = parser.parse_args(sys.argv[1:])

  if ppp.bookid != None:
    text = getbook(ppp.bookid)
  elif ppp.checkmax: # check max number of book id
    tt = requests.get(url+"books?limit=100000").text
    tt2 = json.loads(tt)
    size = len(tt2)
    print(size)
  else: ## random
    idx = random.randint(0, ppp.max - 1)
    tt = requests.get(url+"books?limit=1&skip="+str(idx)).text
    tt2 = json.loads(tt)
    bid = tt2[0]['book_id']
    print("book id:", bid)
    text = getbook(bid)

  if ppp.checkmax:
    pass
  elif ppp.full:
    print(text.encode(ppp.code))
  else:
    summary(text, ppp.code, ppp.num)
    

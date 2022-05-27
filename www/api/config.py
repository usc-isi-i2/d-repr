#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
from pathlib import Path

DB_NAME = os.environ['DB_NAME']
DB_HOST = os.environ['DB_HOST']
DB_USER = os.environ['DB_USER']
DB_PWD = os.environ['DB_PWD']

MSG_QUEUE_HOST = os.environ['MSG_QUEUE_HOST']

ROOT_DIR: Path = Path(__file__).parent.parent
HOME_DIR: Path = ROOT_DIR / os.environ['HOME_DIR']

TIME_OUT = 30  # 30 seconds
EXEC_TIME_OUT = 25
ACK_TIME_OUT = 3
#!/bin/bash

mongo --host mongodb:27017 <<EOF
rs.initiate();
EOF
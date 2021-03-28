#!/bin/bash
# 
# Description:
#
#   Update Earth orientation and space weather data
#
# Usage: 
#   $ ./update_data.sh
# 

# IERS Earth Orientation Data
curl -L https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt -o ./rastro/data/iau2000A_finals_ab.txt
curl -L https://datacenter.iers.org/data/latestVersion/224_EOP_C04_14.62-NOW.IAU2000A224.txt -o ./rastro/data/iau2000A_c04_14.txt

# Space Weather Data
curl -L https://celestrak.com/SpaceData/sw19571001.txt -o ./rastro/data/sw19571001.txt
curl -L ftp://ftp.seismo.nrcan.gc.ca/spaceweather/solar_flux/daily_flux_values/fluxtable.txt -o ./rastro/data/fluxtable.txt

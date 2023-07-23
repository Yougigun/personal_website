#!/bin/bash
docker-compose -f docker-compose.prod.yml up -d --build --force-recreate --remove-orphans

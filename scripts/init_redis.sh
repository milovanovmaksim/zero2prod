# Launch Redis using Docker
docker run \
-p "6379:6379" \
-d \
--name "redis_zero2prod" \
redis:6
echo "Redis is ready to go!"

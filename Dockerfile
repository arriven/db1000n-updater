FROM alpine:3.16
COPY db1000n-updater /usr/local/bin
CMD ["db1000n-updater"]
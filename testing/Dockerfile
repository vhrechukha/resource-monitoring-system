FROM alpine:latest

RUN apk --no-cache add curl bash

COPY load_test.sh /load_test.sh
RUN chmod +x /load_test.sh

CMD ["./load_test.sh"]

codesign --sign "Apple Development: loonlady8@gmail.com (SP63Y5698A)" \
 --timestamp \
 --options runtime \
 -f \
 ./target/release/ntpclient


codesign -dv --verbose=4 ./target/release/ntpclient  # 查看签名信息
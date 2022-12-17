pnpm run compile
rm out.zip
cd out
#rm sdk-*.js
zip ../out.zip *.js
cd ..
aws lambda update-function-code --function-name checkAPS --zip-file fileb://out.zip
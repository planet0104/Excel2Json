cargo build --target armv7-linux-androideabi --release
::cargo build --target aarch64-linux-android --release
::cargo build --target i686-linux-android --release
copy target\armv7-linux-androideabi\release\libexceltojson.so ..\..\jniLibs\armeabi-v7a\libexceltojson.so
::copy target\aarch64-linux-android\release\libexceltojson.so ..\..\jniLibs\arm64-v8a\libexceltojson.so
::copy target\i686-linux-android\release\libexceltojson.so ..\..\jniLibs\x86\libexceltojson.so
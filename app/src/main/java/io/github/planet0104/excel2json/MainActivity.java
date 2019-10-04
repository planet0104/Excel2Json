package io.github.planet0104.excel2json;

import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;

import android.content.pm.PackageManager;
import android.os.Bundle;
import android.os.Environment;
import android.util.Log;
import android.view.View;
import android.widget.TextView;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        verifyStoragePermissions();
        convert();
    }

    private void convert(){
        File file = new File(Environment.getExternalStorageDirectory(), "file.xlsx");
        String path = file.getAbsolutePath();
        Log.d("文件", "file="+path);
        String json = ExcelToJson.convert(path);
        try {
            JSONObject obj = new JSONObject(json);
            Log.d("JSON", obj.toString());
            TextView textJson = findViewById(R.id.textJson);
            textJson.setText(obj.toString());
        } catch (JSONException e) {
            e.printStackTrace();
        }
    }

    public void verifyStoragePermissions() {
        try {
            //检测是否有写的权限
            int permission = ActivityCompat.checkSelfPermission(this,
                    "android.permission.READ_EXTERNAL_STORAGE");
            if (permission != PackageManager.PERMISSION_GRANTED) {
                // 没有写的权限，去申请写的权限，会弹出对话框
                ActivityCompat.requestPermissions(this, new String[]{"android.permission.READ_EXTERNAL_STORAGE"}, 1);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}

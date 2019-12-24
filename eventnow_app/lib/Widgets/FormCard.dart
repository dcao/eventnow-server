import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';

class FormCard extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return new Container(
        width: ScreenUtil.getInstance().setWidth(675),
        height: ScreenUtil.getInstance().setHeight(475),
        decoration: BoxDecoration(
            color: Colors.white,
            borderRadius: BorderRadius.circular(8.0),
            boxShadow: [
              BoxShadow(
                  color: Colors.black12,
                  offset: Offset(0.0, -10.0),
                  blurRadius: 10.0),
            ]),
        child: Padding(
            padding: EdgeInsets.all(16.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                Text(
                  "Login",
                  style: TextStyle(
                      fontSize: ScreenUtil.getInstance().setSp(40),
                      fontFamily: 'Poppins-Bold',
                      letterSpacing: .6),
                ),
                SizedBox(
                  height: ScreenUtil.getInstance().setHeight(30),
                ),
                Text("Username",
                    style: TextStyle(
                      fontSize: ScreenUtil.getInstance().setSp(30),
                      fontFamily: 'Poppins-Bold',
                      letterSpacing: .4,
                    )),
                TextField(
                  decoration: InputDecoration(
                    hintText: "username",
                    hintStyle: TextStyle(color: Colors.grey, fontSize: 12.0),
                  ),
                ),
                SizedBox(
                  height: ScreenUtil.getInstance().setHeight(20),
                ),
                Text("Password",
                    style: TextStyle(
                      fontSize: ScreenUtil.getInstance().setSp(30),
                      fontFamily: 'Poppins-Bold',
                      letterSpacing: .4,
                    )),
                TextField(
                  decoration: InputDecoration(
                    hintText: "password",
                    hintStyle: TextStyle(color: Colors.grey, fontSize: 12.0),
                  ),
                ),
                SizedBox(
                  height: ScreenUtil.getInstance().setHeight(30),
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: <Widget>[
                    Text("Forgot Password?",
                        style: TextStyle(
                          fontSize: ScreenUtil.getInstance().setSp(20),
                          fontFamily: 'Poppins-Bold',
                          color: Colors.blue,
                        ))
                  ],
                )
              ],
            )));
  }
}

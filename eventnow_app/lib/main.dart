import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'Widgets/FormCard.dart';

void main() => runApp(MaterialApp( 
  home: Login(),
));

class Login extends StatefulWidget { 
  @override 
  _LoginState createState() => new _LoginState();
}

class _LoginState extends State<Login> {
  @override 
  Widget build(BuildContext context) { 
    ScreenUtil.instance = ScreenUtil.getInstance()..init(context);
    ScreenUtil.instance = ScreenUtil(width: 750,height: 1334,allowFontScaling: true);
    return new Scaffold(  
      backgroundColor: Colors.yellowAccent,
      resizeToAvoidBottomInset: false,
      body: Stack( 
        fit: StackFit.expand,
        children: <Widget>[ 
          Column(
            children: <Widget>[ 
              Container( 
                padding: new EdgeInsets.all(30.0),
                child: Image.asset('lib/assets/pikachu.png'),
              ),
              SizedBox( 
                height: ScreenUtil.getInstance().setHeight(30),
              ),
              FormCard(),
          ],)
        ],
      )
    );
  }
}



/*
Widget radioButton(bool isSelected) => Container( 
  width: 16.0,
  height: 16.0,
  padding: EdgeInsets.all(2.0),
  decoration: BoxDecoration( 
    shape: BoxShape.circle,
    border: Border.all(width: 2.0, color: Colors.black)),
  child: isSelected 
    ? Container( 
      width: double.infinity,
      height: double.infinity,
      decoration: 
        BoxDecoration(shape: BoxShape.circle, color: Colors.black),
    )
  :Container(), //F
    );
  )
)
*/


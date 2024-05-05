public class StringStartEnd03 {
  public static void test(String str1, String str2, String str3, String str4) {
    String[] strings = new String[4];
    strings[0] = str1;
    strings[1] = str2;
    strings[2] = str3;
    strings[3] = str4;

    int i = 0;
    for (String string : strings) {
      if (string.endsWith("ed")) ++i;
    }
    assert i == 3;
  }
}

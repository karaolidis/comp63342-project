public class StringStartEnd01 {
  public static void test(String[] strings) {
    int i = 0;
    for (String string : strings) {
      if (string.startsWith("te")) ++i;
    }
    assert i == 2;

    i = 0;
    for (String string : strings) {
      if (string.startsWith("ste", 2)) ++i;
    }
    assert i == 1;

    i = 0;
    for (String string : strings) {
      if (string.endsWith("ed")) ++i;
    }
    assert i == 2;
  }
}

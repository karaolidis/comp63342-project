public class StaticCharMethods01 {
  public static void test(char c) {
    assert Character.isDefined(c) == true;
    assert Character.isDigit(c) == false;
    assert Character.isJavaIdentifierStart(c) == false;
    assert Character.isJavaIdentifierPart(c) == true;
    assert Character.isLetter(c) == false;
    assert Character.isLetterOrDigit(c) == false;
    assert Character.isLowerCase(c) == false;
    assert Character.isUpperCase(c) == false;
    assert Character.toUpperCase(c) == Character.toLowerCase(c);
  }
}

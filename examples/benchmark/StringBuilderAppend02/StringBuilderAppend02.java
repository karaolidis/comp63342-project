public class StringBuilderAppend02 {
  public void test(Object objectRef, String string, char[] charArray, boolean booleanValue, char characterValue, int integerValue, long longValue, float floatValue, double doubleValue) {
    StringBuilder lastBuffer = new StringBuilder("last buffer");
    StringBuilder buffer = new StringBuilder();

    buffer
        .append(objectRef)
        .append("%n")
        .append(string)
        .append("%n")
        .append(charArray)
        .append("%n")
        .append(charArray, 0, 3)
        .append("%n")
        .append(booleanValue)
        .append("%n")
        .append(characterValue)
        .append("%n")
        .append(integerValue)
        .append("%n")
        .append(longValue)
        .append("%n")
        .append(floatValue)
        .append("%n")
        .append(doubleValue)
        .append("%n")
        .append(lastBuffer);

    String tmp = buffer.toString();
    assert tmp.equals("diffblue%ntest%nverification%nver%ntrue%n%Z%n7%n10000000000%n2.5%n33.333%nlast buffer");
  }
}

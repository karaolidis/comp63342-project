public class StringValueOf01 {
    public static void test(char[] charArray, boolean booleanValue, char characterValue, int integerValue, long longValue, float floatValue, double doubleValue, Object objectRef) {
        String tmp = String.valueOf(charArray);
        assert tmp.equals("diffblue");

        tmp = String.valueOf(charArray, 3, 3);
        assert tmp.equals("fbl");

        tmp = String.valueOf(booleanValue);
        assert tmp.equals("false");

        tmp = String.valueOf(characterValue);
        assert tmp.equals("T");

        tmp = String.valueOf(integerValue);
        assert tmp.equals("7");

        tmp = String.valueOf(longValue);
        assert tmp.equals("10000000000");

        tmp = String.valueOf(floatValue);
        assert tmp.equals("2.5");

        tmp = String.valueOf(doubleValue);
        assert tmp.equals("33.333");

        tmp = String.valueOf(objectRef);
        assert tmp.equals("test");
    }
}

class Deserializer
{
    private MemoryStream stream;

    public Deserializer(byte[] compiled) {
        stream = new MemoryStream(compiled);
    }
}
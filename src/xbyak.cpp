#include <xbyak/xbyak.h>

class XbyakCore : public Xbyak::CodeGenerator
{
public:
    XbyakCore() {}

    void _mov(int n)
    {
        mov(rax, n);
    }

    void _ret()
    {
        ret();
    }
};

extern "C" XbyakCore *_new()
{
    return new XbyakCore();
}
extern "C" int _gen_code(XbyakCore x)
{
    auto (*func)() = x.getCode<int (*)()>();
    return func();
}
extern "C" void _delete(XbyakCore *x)
{
    delete x;
}

extern "C" void _mov(XbyakCore x, int n)
{
    x._mov(n);
}
extern "C" void _ret(XbyakCore x)
{
    x._ret();
}

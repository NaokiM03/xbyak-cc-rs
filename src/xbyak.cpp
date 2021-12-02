#include <xbyak/xbyak.h>

class XbyakCore : public Xbyak::CodeGenerator
{
public:
    Xbyak::Reg64 reg_rax;

    Xbyak::Reg64 reg_from_int(int n)
    {
        switch (n)
        {
        case 0:
            return reg_rax;
        default:
            throw "Error: failed to parse int to Xbyak::Reg64";
        }
    }

    XbyakCore()
    {
        reg_rax = rax;
    }

    void _mov(Xbyak::Reg64 reg, int n)
    {
        mov(reg, n);
    }
    void _mov(Xbyak::Reg64 reg1, Xbyak::Reg64 reg2)
    {
        mov(reg1, reg2);
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

extern "C" void _mov_r_i(XbyakCore x, int reg, int n)
{
    x._mov(x.reg_from_int(reg), n);
}
extern "C" void _mov_r_r(XbyakCore x, int reg1, int reg2)
{
    x._mov(x.reg_from_int(reg1), x.reg_from_int(reg2));
}
extern "C" void _ret(XbyakCore x)
{
    x._ret();
}

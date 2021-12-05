#include <xbyak/xbyak.h>

class XbyakCore : public Xbyak::CodeGenerator
{
public:
    Xbyak::Reg64 reg_from_int(int n)
    {
        switch (n)
        {
        case 0:
            return rax;
        case 1:
            return rdi;
        default:
            throw "Error: failed to parse int to Xbyak::Reg64";
        }
    }

    XbyakCore() {}

    void _mov(Xbyak::Reg64 reg, int n)
    {
        mov(reg, n);
    }
    void _mov(Xbyak::Reg64 reg1, Xbyak::Reg64 reg2)
    {
        mov(reg1, reg2);
    }

    void _push(int n)
    {
        push(n);
    }
    void _push(Xbyak::Reg64 reg)
    {
        push(reg);
    }
    void _pop(Xbyak::Reg64 reg)
    {
        pop(reg);
    }

    void _add()
    {
        add(rax, rdi);
    }
    void _sub()
    {
        sub(rax, rdi);
    }
    void _mul()
    {
        imul(rax, rdi);
    }
    void _div()
    {
        cqo();
        idiv(rdi);
    }

    void _cmp()
    {
        cmp(rax, rdi);
    }
    void _sete()
    {
        sete(al);
    }
    void _setne()
    {
        setne(al);
    }
    void _setl()
    {
        setl(al);
    }
    void _setle()
    {
        setle(al);
    }
    void _movzx()
    {
        movzx(rax, al);
    }

    void _ret()
    {
        ret();
    }
};

extern "C"
{
    XbyakCore *_new()
    {
        return new XbyakCore();
    }
    int _gen_code(XbyakCore x)
    {
        auto (*func)() = x.getCode<int (*)()>();
        return func();
    }
    void _delete(XbyakCore *x)
    {
        delete x;
    }

    void _mov_r_i(XbyakCore x, int reg, int n)
    {
        x._mov(x.reg_from_int(reg), n);
    }
    void _mov_r_r(XbyakCore x, int reg1, int reg2)
    {
        x._mov(x.reg_from_int(reg1), x.reg_from_int(reg2));
    }

    void _push_i(XbyakCore x, int n)
    {
        x._push(n);
    }
    void _push_r(XbyakCore x, int reg)
    {
        x._push(x.reg_from_int(reg));
    }
    void _pop(XbyakCore x, int reg)
    {
        x._pop(x.reg_from_int(reg));
    }

    void _add(XbyakCore x)
    {
        x._add();
    }
    void _sub(XbyakCore x)
    {
        x._sub();
    }
    void _mul(XbyakCore x)
    {
        x._mul();
    }
    void _div(XbyakCore x)
    {
        x._div();
    }

    void _cmp(XbyakCore x)
    {
        x._cmp();
    }
    void _sete(XbyakCore x)
    {
        x._sete();
    }
    void _setne(XbyakCore x)
    {
        x._setne();
    }
    void _setl(XbyakCore x)
    {
        x._setl();
    }
    void _setle(XbyakCore x)
    {
        x._setle();
    }
    void _movzx(XbyakCore x)
    {
        x._movzx();
    }

    void _ret(XbyakCore x)
    {
        x._ret();
    }
}

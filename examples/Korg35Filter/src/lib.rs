use chugin;
use chugin::chuck;

use dspz::types::Float;
use dspz::filter::korg35::Korg35;
use dspz::traits::Processor;

static mut DATA_OFFSET: usize = 0;

chugin::ctor!(ctor, DATA_OFFSET, {
    let obj = Korg35::new(44100.0);
    obj
});

chugin::dtor!(dtor, DATA_OFFSET, Korg35, _obj, {});

chugin::tick!(tick, DATA_OFFSET, Korg35, obj, inp, { 
    obj.tick(inp as Float) as f32
});

chugin::mfun_setter_getter_float!(
    set_freq,
    get_freq,
    DATA_OFFSET,
    Korg35,
    k35,
    freq,
    {
        k35.set(freq as Float, k35.get_K() );
    },
    { k35.get_freq() }
);

chugin::mfun_setter_getter_float!(
    set_k,
    get_k,
    DATA_OFFSET,
    Korg35,
    k35,
    k,
    {
        k35.set(k35.get_freq(), k as Float );
    },
    { k35.get_K() }
);

fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;

    q.begin_class("Korg35", "UGen")?;

    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;

    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { DATA_OFFSET = offset; }

    q.add_ugen_func(Some(tick), 1, 1)?;

    q.add_mfun(
        Some(set_freq),
        "float",
        "freq",
        &[(String::from("float"), String::from("f"))],
    )?;

    q.add_mfun(
        Some(set_k),
        "float",
        "K",
        &[(String::from("float"), String::from("K"))],
    )?;

    q.end_class()?;

    Ok(())
}

chugin::query!(query, ck_query_impl(query));

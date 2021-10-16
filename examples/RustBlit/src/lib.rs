use chugin;
use chugin::chuck;
use dspz::types::Float;
use dspz::oscs::blit::Blit;
use dspz::traits::{Generator,Periodic};

static mut DATA_OFFSET: usize = 0;

chugin::ctor!(ctor, DATA_OFFSET, {
    let obj = Blit::new(44100.0);
    obj
});

chugin::dtor!(dtor, DATA_OFFSET, Blit, _obj, {});

chugin::mfun_setter_getter_float!(
    set_freq,
    get_freq,
    DATA_OFFSET,
    Blit,
    blit,
    freq,
    {
        blit.set_freq(freq as Float);
    },
    { blit.get_freq() }
);

chugin::tick!(tick, DATA_OFFSET, Blit, obj, _inp, { 
    obj.tick() as f32
});

fn ck_query_impl(query: *mut chuck::DL_Query) -> chugin::CKResult {
    let q = chugin::Query::new(query)?;

    q.begin_class("RustBlit", "UGen")?;

    q.add_ctor(Some(ctor))?;
    q.add_dtor(Some(dtor))?;

    let offset = q.add_mvar("int", "@data", false)? as usize;
    unsafe { DATA_OFFSET = offset; }

    q.add_ugen_func(Some(tick), 0, 1)?;

    q.add_mfun(
        Some(set_freq),
        "float",
        "freq",
        &[(String::from("float"), String::from("f"))],
    )?;

    q.end_class()?;

    Ok(())
}

chugin::query!(query, ck_query_impl(query));

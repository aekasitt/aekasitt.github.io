/* ~~/src/pages/learn.rs */

// third-party crates
use leptos::prelude::*;

// local modules
use crate::components::ui::badge::{Badge, BadgeSize, BadgeVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::icons::Check;

#[component]
fn CurriculumCard(
    #[prop(into)] eyebrow: String,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] lessons: String,
) -> impl IntoView {
    view! {
        <Card class="h-full overflow-hidden relative">
            <div class="absolute bg-primary h-1 inset-x-0 top-0" />
            <CardHeader>
                <div class="flex items-center justify-between gap-4 mb-3">
                    <Badge variant=BadgeVariant::Secondary size=BadgeSize::Sm>
                        {eyebrow}
                    </Badge>
                    <span class="text-muted-foreground text-xs uppercase tracking-[0.16em]">
                        Members
                    </span>
                </div>
                <CardTitle class="font-bold leading-tight text-xl">
                    {title}
                </CardTitle>
                <CardDescription class="leading-relaxed pt-2">
                    {description}
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div class="border-t flex items-center justify-between pt-4 text-sm">
                    <span class="text-muted-foreground">{lessons}</span>
                    <span class="font-medium text-primary">Locked</span>
                </div>
            </CardContent>
        </Card>
    }
}

#[component]
pub fn Learn() -> impl IntoView {
    let benefits = [
        "Long-form, implementation-first courses",
        "Complete source code and downloadable notes",
        "New Rust and Bitcoin material every month",
        "Direct support for independent technical writing",
    ];

    view! {
        <section class="lg:px-16 pb-20 px-8 pt-8">
            <div class="border overflow-hidden relative rounded-2xl">
                <div class="absolute bg-primary/10 blur-3xl h-72 -right-24 -top-24 w-72" />
                <div class="gap-10 grid lg:grid-cols-[1.25fr_0.75fr] p-8 relative sm:p-12">
                    <div class="max-w-3xl">
                        <Badge variant=BadgeVariant::Secondary size=BadgeSize::Lg>
                            {"Guru's Gazette Premium"}
                        </Badge>
                        <h1 class="font-bold leading-[1.05] mt-6 text-4xl tracking-tight sm:text-6xl">
                            Go beyond the feed.
                        </h1>
                        <p class="leading-relaxed mt-5 text-lg text-muted-foreground sm:text-xl">
                            Deep, practical learning for people building with Rust, Bitcoin,
                            and the systems around them.
                        </p>
                        <div class="flex flex-col gap-3 mt-8 sm:flex-row">
                            <a
                                class="bg-primary font-medium hover:bg-primary/90 px-6 py-3 rounded-md text-center text-primary-foreground transition-colors"
                                href="https://geyser.fund/project/gazette"
                                rel="noreferrer"
                                target="_blank"
                            >
                                Become a member
                            </a>
                            <a
                                class="border font-medium hover:bg-accent px-6 py-3 rounded-md text-center transition-colors"
                                href="https://geyser.fund/project/gazette"
                                rel="noreferrer"
                                target="_blank"
                            >
                                View sponsorship
                            </a>
                        </div>
                    </div>

                    <div class="bg-card border p-6 rounded-xl shadow-sm">
                        <p class="font-semibold text-lg">Inside the membership</p>
                        <ul class="list-none mt-4 p-0 space-y-4">
                            {benefits.into_iter().map(|benefit| view! {
                                <li class="flex gap-3 items-start">
                                    <span class="bg-secondary flex items-center justify-center mt-0.5 rounded-full shrink-0 size-6 text-primary">
                                        <Check class="size-3" />
                                    </span>
                                    <span class="leading-relaxed text-sm">{benefit}</span>
                                </li>
                            }).collect_view()}
                        </ul>
                    </div>
                </div>
            </div>

            <div class="mt-14">
                <div class="max-w-2xl">
                    <p class="font-semibold text-primary text-sm uppercase tracking-[0.18em]">
                        Premium curriculum
                    </p>
                    <h2 class="font-bold mt-3 text-3xl tracking-tight">
                        Learn by building the real thing
                    </h2>
                    <p class="mt-3 text-muted-foreground">
                        Each path combines careful explanations, production-minded trade-offs,
                        and a project you can keep.
                    </p>
                </div>
                <div class="gap-4 grid md:grid-cols-3 mt-8">
                    <CurriculumCard
                        eyebrow="Rust"
                        title="Production Rust, from first principles"
                        description="Own the concepts that make Rust reliable: ownership, APIs, async systems, and observability."
                        lessons="12 lessons"
                    />
                    <CurriculumCard
                        eyebrow="Bitcoin"
                        title="Bitcoin protocols in practice"
                        description="Move from transactions and Script to modern protocol design with executable examples."
                        lessons="9 lessons"
                    />
                    <CurriculumCard
                        eyebrow="Systems"
                        title="Fast software, measured properly"
                        description="Profile real workloads, reason about memory, and turn benchmark results into sound decisions."
                        lessons="8 lessons"
                    />
                </div>
            </div>
            <div class="bg-muted border mt-14 p-7 rounded-xl sm:flex sm:items-center sm:justify-between sm:p-9">
                <div>
                    <h2 class="font-bold text-2xl">Already a member?</h2>
                    <p class="mt-2 text-muted-foreground">
                        Contact the author from the email used for your contribution to activate access.
                    </p>
                </div>
                <a
                    class="border bg-background inline-flex font-medium hover:bg-accent mt-5 px-5 py-2.5 rounded-md transition-colors sm:mt-0"
                    href="https://geyser.fund/project/gazette"
                    rel="noreferrer"
                    target="_blank"
                >
                    Verify membership
                </a>
            </div>
        </section>
    }
}

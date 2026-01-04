/**
 * DocLink component - Link to documentation with icon and description
 */

interface DocLinkProps {
  title: string;
  description: string;
  href: string;
  external?: boolean;
  icon?: string;
}

export function DocLink({ title, description, href, external = false, icon = '📄' }: DocLinkProps) {
  return (
    <a
      href={href}
      target={external ? '_blank' : undefined}
      rel={external ? 'noopener noreferrer' : undefined}
      className="card hover:border-cyan-electric/60 hover:shadow-glow-cyan transition-all duration-200 cursor-pointer block"
    >
      <div className="flex items-start gap-4">
        <span className="text-3xl">{icon}</span>
        <div className="flex-1">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">{title}</h3>
          <p className="text-body text-light-gray">{description}</p>
          {external && (
            <span className="inline-block mt-2 text-body-sm text-medium-gray">
              External link →
            </span>
          )}
        </div>
      </div>
    </a>
  );
}
